'''
    COMMAND LINE SUPPORT
'''
def __executecmd(cwd, args):
    p = subprocess.Popen(args, cwd = cwd, stdout=subprocess.PIPE, stderr=subprocess.PIPE, stdin=subprocess.PIPE)
    return (p.stdout.read().decode('utf-8'), p.stderr.read().decode('utf-8'))
    
class bcolors:
    HEADER = '\033[95m'
    OKBLUE = '\033[94m'
    OKGREEN = '\033[92m'
    WARNING = '\033[93m'
    FAIL = '\033[91m'
    ENDC = '\033[0m'
    
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
    
    if len(no.strip()) > 0:
        print(bcolors.HEADER + bcolors.WARNING + no + bcolors.ENDC)
    
    if len(se) > 0:
        return False
    return True
