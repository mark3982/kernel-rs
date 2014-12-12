import sys
import os
import argparse
import importlib.machinery
import imp
from pymake.lib import *

_print = print
def print(s):
    _print(s + bcolors.ENDC)
def printerror(s):
    print(bcolors.FAIL + 'error: ' + bcolors.ENDC + s)
def printwarn(s):
    print(bcolors.WARNING + 'error: ' + bcolors.ENDC + s)
def fail(s):
    printerror(s)
    exit()

def showboards():
    showdirofdesc('./boards')

def showtargets():
    showdirofdesc('./targets')

class Tool:
    def __init__(self, name, path):
        self._name = name
        self._path = path
    def use(self, wdir, args, cmdshow):
        code, stderr = executecmd(wdir, '%s %s' % (self._path, args), cmdshow=cmdshow)
        if code is False:
            #
            # On my system I have a problem with virtual-box shares and unlink operation,
            # so it errors although it is harmless therefore I detect this condition.
            # -- kmcg3413@gmail.com 12/12/2014
            #
            if self._name == 'rustc':
                stderr = stderr.strip()
                if stderr.find('error:') == 0 and stderr.count('\n') < 2 and stderr.find('unlink path (Text file busy;') > -1:
                    # harmless error, just ignore it
                    return

            printerror('tool usage failed with:')
            print('DIR:  ' + wdir)
            print('TOOL: ' + self._path)
            print('ARGS: ' + args)
            print('OUTPUT:')
            print(bcolors.DIM + stderr)
            exit()

class Tools:
    def __init__(self):
        self._tools = {}
    def add(self, name, path, desc = None):
        self._tools[name] = Tool(name, path)
    def __getattr__(self, attr):
        if attr not in self._tools or self._tools[attr] is None:
            fail('not told about or unable to find tool `%s` but required!' % attr)
        return self._tools[attr]


def build(args, wdir = None, sdir = None):
    # make sure the board and target are valid
    if args.target is None:
        return printerror('You must specify --target=TARGET, try passing `targets` for action to list targets.')
    if args.board is None:
        return printerror('You must specify --board=BOARD, try passing `boards` for action to list boards.')
    if not os.path.exists('./boards/' + args.board):
        return printerror('The board `%s` does not exist!' % args.board)
    if not os.path.exists('./targets/' + args.target):
        return printerror('The target `%s` does not exist!' % args.target)

    wdir = wdir or os.environ['PWD']
    sdir = sdir or os.environ['PWD']

    wdir = sdir + '/build/'

    showcmd = args.showcommands or False

    # we need to locate our tools unless they were specified
    # on the command line and in that case use the specified
    # tool, if a tool is not found the build can continue unless
    # the tool is used..
    print('looking for any tools that can be found and were not specified..')
    tools = Tools()
    tools.add('rustc', args.rustc or locatetool('rustc'))
    tools.add('ar', args.ar or locatetool('ar'))
    tools.add('ld', args.ld or locatetool('ld'))
    tools.add('objcopy', args.objcopy or locatetool('objcopy'))
    tools.add('gas', args.gas or locatetool('as'))
    tools.add('gcc', args.gcc or locatetool('gcc'))
    tools.add('cp', args.cp or locatetool('cp'))

    # now we have our tools we can try to actually build
    print('building board `' + bcolors.OKGREEN + args.board + bcolors.ENDC + '` for ' + bcolors.OKGREEN + args.target + '..')

    # Load board python module and target python module if they exist
    # so that we can hook into them for custom things that need to be
    # done during the build; we should never make board or target specific
    # decisions here unless we use the output of the hook and therefore
    # it would apply to all targets and boards
    boardpyhookmod = '%s/boards/%s/makehook.py' % (sdir, args.board)
    if os.path.exists(boardpyhookmod):
        # does not support paths
        #boardhookmod = __import__(boardpyhookmod)
        # 3.3+
        #loader = importlib.machinery.SourceFileLoader('boardpyhook', boardpyhookmod)
        #boardhookmod = loader.load_module()
        # 3.2
        boardhookmod = imp.load_source('boardpyhook', boardpyhookmod)
    else:
        class DummyBoardHooks:
            def hook_membaseget(args):
                return {'result': args['membase']}
        boardhookmod = DummyBoardHooks()

    targetpyhookmod = '%s/targets/%s/makehook.py' % (sdir, args.target)
    if os.path.exists(targetpyhookmod):
        targethookmod = imp.load_source('targetpybook', targetpyhookmod)
    else:
        class DummyTargetHooks:
            def hook_prelinkforobjectfiles(args):
                return {'result': []}
        targethookmod = DummyTargetHooks()

    #
    # You may notice that the hooks take a dictionary and return a dictionary
    # and this is done to allow flexibility with out incurring a lot of rewrite
    # to existing function just to add more arguments. Also the return is a dictionary
    # which also helps to keep from having to do a rewrite. So instead of checking
    # for and calling a different version function a newer version could just return
    # newer information, or set a certain flag indicating something different. Now
    # just because this is possible does not mean it should be used, but in most cases
    # it could be fairly common to add extra hook arguments in and therefore not
    # have to change functions or create new function versions but instead just allow
    # the newer function to take advantage of newer arguments. -- kmcg3413@gmail.com 12/12/14
    #

    # if the board has a hook then let it decide what to use
    membase = boardhookmod.hook_membaseget({'membase': args.membase})['result']

    # if working directory not the same as source directory then
    # we need to copy our dummy libs there so they will be picked
    # up and used
    if wdir != sdir:
        tools.cp.use(wdir, '%s/libmorestack.a %s/' % (sdir, wdir), showcmd)
        tools.cp.use(wdir, '%s/libcompiler-rt.a %s/' % (sdir, wdir), showcmd)

    # Let us build the rust side of everything
    #$TOOL_RUSTC -C relocation-model=static -C no-stack-check --crate-type rlib ./core/core.rs --target=$ARCH
    #$TOOL_RUSTC -C relocation-model=static -C no-stack-check --extern core=libcore.rlib --crate-type rlib ./boards/$BOARD/board.rs --target=$ARCH
    #$TOOL_RUSTC -C relocation-model=static --crate-type staticlib -C no-stack-check -L . __main.rs --opt-level 3 --target=$ARCH
    src = '%s/core/core.rs' % sdir
    tools.rustc.use(wdir, '-C relocation-model=static -C no-stack-check --crate-type rlib %s --target=%s' % (src, args.target), showcmd)
    src = '%s/boards/%s/board.rs' % (sdir, args.board)
    tools.rustc.use(wdir, '-C relocation-model=static -C no-stack-check --extern core=libcore.rlib --crate-type rlib %s --target=%s' % (src, args.target), showcmd)
    src = '%s/__main.rs' % sdir
    tools.rustc.use(wdir, '-C relocation-model=static -C no-stack-check --crate-type staticlib -L . --opt-level 3 %s --target=%s' % (src, args.target), showcmd)

    #
    # We need to get all the object files out and link
    # them together which will resolve all the symbol
    # references between them and produce a final object
    # file which should have no relocations.
    #
    #
    # This output is likely going to be ELF but it depends
    # on your LD tool as it will produce whatever it was
    # compiled or specified to produce.
    #
    #
    # If you produce an ELF it can be used by QEMU depending
    # on the architecture you are emulating. 
    #
    #$TOOL_AR xvf lib__main.a
    #rm *-test.o
    #$TOOL_LD *.o $LDOPTS -o kernel.elf -Ttext $MEM_BASE
    tools.ar.use(wdir, 'xvf lib__main.a', showcmd)

    # hook for inclusion of any architecture specific files
    targethookmod.hook_prelinkforobjectfiles({
        'cmdlineargs': args, 'tools': tools, 'wdir': wdir, 'sdir': sdir,
        'target': args.target, 'board': args.board
    })

    # the way we are handling arguments *.o does not expand 
    _objfiles = enumfilesbyext(wdir, 'o')
    # we need to yank out the fake files
    objfiles = []
    for objfile in _objfiles:
        if objfile.startswith('r-compiler-rt-') or objfile.startswith('r-morestack-'):
            continue
        _print(objfile)
        objfiles.append(objfile)
    objfiles = ' '.join(objfiles)
    tools.ld.use(wdir, '%s -o kernel.elf -Ttext %s' % (objfiles, membase), showcmd)

    #
    # If you need a flat binary then this will do the
    # trick and can be flashed to a ROM if desired. However
    # you may want to determine how you want to handle the
    # `.data` and `.rodata` sections otherwise you will find
    # bugs.
    #
    #echo $TOOL_OBJCOPY -j .text -O binary $OUTBASENAME.elf $OUTBASENAME.bin
    tools.objcopy.use(wdir, '-j .text -O binary kernel.elf kernel.bin', showcmd)


def cli():
    parser = argparse.ArgumentParser(description='kernel-rs build system', epilog='Try build!')
    parser.add_argument('-target', help='target architecture')
    parser.add_argument('-board', help='target board')
    parser.add_argument('-rustc', help='path to the Rust language compiler')
    parser.add_argument('-ar', help='path to the binutils archive tool')
    parser.add_argument('-ld', help='path to the binutils linker tool')
    parser.add_argument('-objcopy', help='path to the binutils objcopy tool')
    parser.add_argument('-gas', help='path to the binutils assembler tool')
    parser.add_argument('-gcc', help='path to the GCC compiler')
    parser.add_argument('action', help='must be "boards", "targets", or "build"')
    parser.add_argument('-showcommands', action='store_const', const=True, help='show all shell commands that are executed')
    parser.add_argument('-membase', help='memory address to base image if not position independant')
    parser.add_argument('-cp', help='path to copy(cp) if need to specify')
    args = parser.parse_args()

    # help them figure out what to do
    if args.action not in ('boards', 'targets', 'build'):
        printerror('error: `%s` not recognized; must be `boards`, `targets`, or `build`' % args.action) 
        return

    if args.action == 'boards': return showboards()
    if args.action == 'targets': return showtargets()
    if args.action == 'build': return build(args)
    


cli()