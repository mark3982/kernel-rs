'''
    x86universal make hook
'''

def hook_membaseget(args):
    '''
        If memory base not specified then use the default value.
    '''
    return {'result': args['membase'] or '0x100000' }

def hook_prelinkforoptions(args):
    sdir = args['sdir']
    board = args['board']
    return '-T%s/boards/%s/link.ld' % (sdir, board)