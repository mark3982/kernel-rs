from pymake.lib import *

'''
    Here we hook into the build process. 

    1. We need to hook into the prelink in order to build and then
       pass back some files to link that are specific to the ARM
       architecture. To be specific we need to include some object
       files that hold symbols for a few math and memory operations.
'''

def hook_prelinkforobjectfiles(args):
    tools = args['tools']
    wdir = args['wdir']
    sdir = args['sdir']
    target = args['target']
    board = args['board']

    #tools.as.use(wdir, '%s/targets/%s/')

    nodes = os.listdir('%s/targets/%s/' % (sdir, target))
    for node in nodes:
        if node.find('.') > -1 and node[node.find('.') + 1:] == 's':
            objname = node[0:node.find('.')] + '.o'
            tools.gas.use(wdir, '-o %s %s/targets/%s/%s' % (objname, sdir, target, node), args['cmdlineargs'].showcommands)

    return {''}