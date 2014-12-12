import pymake.lib as pymake
import sys
import os
import argparse

class bcolors:
    HEADER = '\033[95m'
    OKBLUE = '\033[94m'
    OKGREEN = '\033[92m'
    WARNING = '\033[93m'
    FAIL = '\033[91m'
    ENDC = '\033[0m'

_print = print
def print(s):
    _print(s + bcolors.ENDC)
def printerror(s):
    print(bcolors.FAIL + 'error: ' + bcolors.ENDC + s)
def printwarn(s):
    print(bcolors.WARNING + 'error: ' + bcolors.ENDC + s)

def listdirfull(dir):
    out = []
    nodes = os.listdir(dir)
    for node in nodes:
        out.append('%s/%s' % (dir, node))
    return out

def showdirofdesc(dir):
    nodes = os.listdir(dir)
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
            print(bcolors.OKGREEN + node.ljust(20) + bcolors.ENDC + '- ' + desc)

def showboards():
    showdirofdesc('./boards')

def showtargets():
    showdirofdesc('./targets')

def build(args):
    # make sure the board and target are valid
    if args.target is None:
        return printerror('You must specify --target=TARGET, try passing `targets` for action to list targets.')
    if args.board is None:
        return printerror('You must specify --board=BOARD, try passing `boards` for action to list boards.')
    if not os.path.exists('./boards/' + args.board):
        return printerror('The board `%s` does not exist!' % args.board)
    if not os.path.exists('./targets/' + args.target):
        return printerror('The target `%s` does not exist!' % args.target)

    print('building board `' + bcolors.OKGREEN + args.board + bcolors.ENDC + '` for ' + bcolors.OKGREEN + args.target)


def cli():
    parser = argparse.ArgumentParser(description='kernel-rs build system', epilog='Try build!')
    parser.add_argument('-target', help='target architecture')
    parser.add_argument('-board', help='target board')
    parser.add_argument('action', help='must be "boards", "targets", or "build"')
    args = parser.parse_args()

    # help them figure out what to do
    if args.action not in ('boards', 'targets', 'build'):
        printerror('error: `%s` not recognized; must be `boards`, `targets`, or `build`' % args.action) 
        return

    if args.action == 'boards': return showboards()
    if args.action == 'targets': return showtargets()
    if args.action == 'build': return build(args)
    


cli()