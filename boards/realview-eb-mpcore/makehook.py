'''
    realview-eb-mpcore make hook
'''

def hook_membaseget(args):
    '''
        If memory base not specified then use the default value.
    '''
    return {'result': args['membase'] or '0x10000' }