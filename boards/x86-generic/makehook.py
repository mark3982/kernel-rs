'''
    x86universal make hook
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
        fail('<unimplemented> x86 universal board does not support specification of link address!')
 
    return {'result': args['membase'] or '0x100000' }

def hook_prelinkforoptions(args):
    sdir = args['sdir']
    board = args['board']
    return '-T%s/boards/%s/link.ld' % (sdir, board)

def hook_prelinkforobjectfiles(args):
    tools = args['tools']
    wdir = args['wdir']
    sdir = args['sdir']
    target = args['target']
    board = args['board']

    nodes = os.listdir('%s/boards/%s/' % (sdir, board))
    for node in nodes:
        if node.find('.') > -1 and node[node.find('.') + 1:] == 's':
            objname = node[0:node.find('.')] + '.o'
            tools.gas.use(wdir, '-o %s %s/boards/%s/%s' % (objname, sdir, board, node), args['cmdlineargs'].showcommands)

    return {}