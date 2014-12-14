'''
    realview-eb-mpcore make hook
'''

import os
from pymake.lib import *

def hook_membaseget(args):
    #
    # The problem here is we need this to be reflected in the linker
    # script so I need to write some code to produce the linker script
    # as a temporary file and have that used during the link phase in
    # the `hook_prelinkforoptions`.
    #
    if args['membase'] is not False:
        fail('<unimplemented> realview-eb-mpcore board does not support specification of link address!')
 
    return {'result': args['membase'] or '0x10000' }

def hook_prelinkforoptions(args):
    sdir = args['sdir']
    board = args['board']
    clargs = args['cmdlineargs']

    return '-T%s/boards/%s/link.ld' % (sdir, board)

def hook_prelinkforobjectfiles(args):
    tools = args['tools']
    wdir = args['wdir']
    sdir = args['sdir']

    target = args['target']
    board = args['board']
    clargs = args['cmdlineargs']

    buildtype = clargs.boardimagetype or 'qemu'

    if buildtype not in ('qemu', 'rom'):
        fail('--boardimagetype must be `qemu` or `rom`')

    if buildtype == 'qemu':
        tools.gas.use(wdir, '-o %s %s/boards/%s/%s' % ('qemuboot.o', sdir, board, 'qemuboot.s'), clargs.showcommands)
    if buildtype == 'rom':
        if not clargs.initialstack:
            fail('for --boardimagetype=rom you must specify --initialstack=<address>') 
        tools.gas.use(wdir, '-o %s %s/boards/%s/%s' % ('romboot.o', sdir, board, 'romboot.s'), clargs.showcommands)

    #nodes = os.listdir('%s/boards/%s/' % (sdir, board))
    #for node in nodes:
    #    if node.find('.') > -1 and node[node.find('.') + 1:] == 's':
    #        objname = node[0:node.find('.')] + '.o'
    #        tools.gas.use(wdir, '-o %s %s/boards/%s/%s' % (objname, sdir, board, node), args['cmdlineargs'].showcommands)

    return {}