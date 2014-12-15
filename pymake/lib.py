import subprocess
import os

'''
    COMMAND LINE SUPPORT
'''
def __executecmd(cwd, args):
    p = subprocess.Popen(args, cwd = cwd, stdout=subprocess.PIPE, stderr=subprocess.PIPE, stdin=subprocess.PIPE)
    return (p.stdout.read().decode('utf-8', 'ignore'), p.stderr.read().decode('utf-8', 'ignore'))

_print = print
def print(s):
    _print(s + bcolors.ENDC)
def printerror(s):
    print(bcolors.FAIL + 'error: ' + bcolors.ENDC + s)
def printwarn(s):
    print(bcolors.WARNING + 'error: ' + bcolors.ENDC + s)
def fail(s = None, nostackdump = False):
    if s is not None:
        printerror(s)
    if nostackdump:
        exit()
    raise Exception('MAKE FAILURE')
    
class bcolors:
    HEADER = '\033[95m'
    OKBLUE = '\033[94m'
    OKGREEN = '\033[92m'
    WARNING = '\033[93m'
    FAIL = '\033[91m'
    ENDC = '\033[0m'
    DIM = '\033[90m'

def listdirfull(dir):
    out = []
    nodes = os.listdir(dir)
    for node in nodes:
        out.append('%s/%s' % (dir, node))
    return out

def showdirofdesc(pad, dir):
    nodes = os.listdir(dir)
    mx = None
    for node in nodes:
        if mx is None or len(node) > mx:
            mx = len(node)
    for node in nodes:
        fnode = '%s/%s' % (dir, node)
        if os.path.isdir(fnode):
            descpath = '%s/desc' % fnode
            if os.path.exists(descpath):
                fd = open(descpath, 'r')
                desc = fd.readline().strip()
                fd.close()
            else:
                desc = ''
            print(pad + bcolors.OKGREEN + node.ljust(mx + 2) + bcolors.ENDC + '- ' + desc)


def isindir(dir, name):
    nodes = os.listdir(dir)
    if name in nodes: return True
    return False

def locatetool(tool):
    env = os.environ
    path = env['PATH']
    parts = path.split(':')
    for part in parts:
        if isindir(part, tool):
            return part + '/' + tool
    return None

def enumfilesbyext(dir, ext):
    out = []
    nodes = os.listdir(dir)
    for node in nodes:
        if node.find('.') < 0:
            if ext is None:
                out.append(node)
            continue
        if node[node.find('.') + 1:] == ext:
            out.append(node)
            continue
    return out

    
def executecmd(cwd, args, cmdshow=True):
    if cmdshow:
        print('\t\t[%s] %s' % (cwd, args))
    args = args.split(' ')

    so, se = __executecmd(cwd, args)
    
    no = []
    parts = se.split('\n')
    for part in parts:
        no.append('\t%s\n' % part)
    no = ''.join(no)
    
    # let calling code decide what to do with stderr
    #if len(no.strip()) > 0:
    #    print(bcolors.HEADER + bcolors.WARNING + no + bcolors.ENDC)
    
    if len(se) > 0:
        return (False, se)
    return (True, se)
