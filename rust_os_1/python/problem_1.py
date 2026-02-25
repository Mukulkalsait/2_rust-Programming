import os 
import sys
import subprocess
import signal
import time

if len(sys.argv) < 2:
    print("Usage: python3 problem_1.py <command> [args...]")
    sys.exit(1)

command = sys.argv[1:]

child_proc = None
running_status = True


# Handling
def handle_termination(signum, stack_frame_current):
    global running_status, child_proc
    print(f"Received signal: {signum}")
    print("Supervisor recieved SIGTERM. Shutting down gracefully...")

    running_status = False

    # comming from Popen()
    # IMP: POLL => it checks if process FINIGHED or not. if yes -> return Exit Code || if not then return None.
    if child_proc and child_proc.poll() is None:
        print(f"Forwarding {signum}(previously SIGTERM) to child PID: {child_proc.pid}")

        os.killpg(child_proc.pid, signal.SIGTERM) 
        # TAG:
        # os.killProcessGroup(with pid, sent SIGTERM 15)
        # killpg(processGroupID, and SIGXXXX)
        # if we dont move the child_proc to background, 
        # the killProcessGroup(function) wil not get the group id from child_proc
        # resualting in roblametic code. 
        # we can also do os.killpg(os.getpgid(child_proc.pid), signal.SIGTERM)
        # this will get the pid from Supervisor and kill eerytin in it. but Dengerous.
        
        # child_proc.terminate() #sends SIGTERM
        try: 
            child_proc.wait(timeout=5)
        except:
            print("Child Process ignored SIGTERM. Forcekilling with SIGKILL")
            os.killpg(child_proc.pid, signal.SIGKILL) 
            # Sigkill will directly kill no anyting return TO THE TARGATED PROCESS BUT still SIGCHLD is sent to the parent.
            child_proc.wait()

    sys.exit(0)

signal.signal(signal.SIGTERM, handle_termination) 
signal.signal(signal.SIGINT, handle_termination) 

# R: this 2 cannot be caughed
# signal.signal(signal.SIGKILL, handle_termination) 
# signal.signal(signal.SIGSTOP, handle_termination) 

# NOTE: 
# provides handler(signum, current_stack_frame) passed to =>  handle_termination().
#  signum =  15 (SIGTERM) 
#  current_stack_frame = stack frame.


# OPENING LOGS 
stdout_log = open("out.log","a")
stderr_log = open("err.log","a")

while running_status: 
    try: 
        print("Starting child process")

        child_proc = subprocess.Popen(command,stdout=stdout_log,stderr=stderr_log,stdin=subprocess.DEVNULL,start_new_session=True)
        # Popen => fork() + exec() + (additional setsid = start_new_session=True)
        # Popen gives us=> wait(),poll(),terminate(),kill().
        # IMP: the start_new_session sayd GO to backgroudn but only for this FUNCITON
        # hence: the main function stays waiting for our signal, and this part run in background.
        # hence commands like sleep (3/5 anyting) will work but cannot be killed
        # because whoel process will go to background and no one will listen to Ctrl + c
        print(f"Started process with pid {child_proc.pid}")

        child_proc.wait() 
        # WAIT() use:
        # 1.block while till child die, so only 1 child otherwise system crash
        # 2.prevent zombie
        # 3.detect exit and provide returncode (automatically stores child_proc.returncode)
        print(f"Child exited with code: {child_proc.returncode}")

        print("Restarting in 1 sec...\n")

        time.sleep(1)

    except Exception as e: 
        print(f"Supervisor error: {e}")
        time.sleep(1)


# ----------------------------------------------------------------------------------------------------
# sob basically: 
# 1st we provide arg => filter into command
# then child_proc =none and runnitn = true
# stdout and erro file open()
#
# while running_status :  Y: 1
#   try 
#   child_proc= subprocess.Popen(cmd, out, err, in, start_new_session)
#   child_proc.wait()
#   time.sleep(1)
#
# except Ex as e: 
#   print fail
#   time.sleep(1)
#
#   if we recieve termination signal
#   signal.signal(signal.SIGTERM,term_function)
#
#   term_funciton(num, stackframe)
#       global child_proc, running_status
#
#       running_status = False => for the while loop avobe G: for => 1
#
#       check if stoped if not stop gressfully.
#       poll.
#       if child_proc and child_proc.poll is None:
#         show signum, child pid
#         child.terminate
#         child.wait()
#       sys.exit()
#
#
# ----------------------------------------------------------------------------------------------------
#       now why child.wait? 
#       why in both places? 
# ----------------------------------------------------------------------------------------------------

