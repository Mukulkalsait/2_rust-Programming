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
max_restart_tries = 3


#----------------------------------------------------
# Termination haldler
def handle_termination(signum, stack_frame_current):
    global running_status
    print(f"Received signal: {signum}")
    running_status = False

signal.signal(signal.SIGTERM, handle_termination) 
signal.signal(signal.SIGINT, handle_termination) 

#----------------------------------------------------
# Child Process loop

stdout_log = open("out.log","a")
stderr_log = open("err.log","a")


while running_status and max_restart_tries >= 0 : 
    max_restart_tries -= 1
    if max_restart_tries <= 0 
        print("Maximum Restarting Limit Reached")
        break
    print("Starting child process")
    child_proc = subprocess.Popen(command,stdout=stdout_log,stderr=stderr_log,stdin=subprocess.DEVNULL,start_new_session=True)
    print(f"Started process with pid {child_proc.pid}")

    time.sleep(3)

    try: 
        child_proc.wait() 
    except KeyboardInterrupt: 
        pass
    # R:Loop does not have wait() so main thread will not sotphere.

#----------------------------------------------------
# G: Child Process terminator which will trigger after loop end & CLEANUP => 

if child_proc and child_proc.poll() is None:
    print(f"Forwarding to child PID: {child_proc.pid}")

    os.killpg(child_proc.pid, signal.SIGTERM) 
    try: 
        child_proc.wait(timeout=5)
    except:
        print("Child Process ignored SIGTERM. Forcekilling with SIGKILL")
        os.killpg(child_proc.pid, signal.SIGKILL) 
        # Sigkill will directly kill no anyting return TO THE TARGATED PROCESS BUT still SIGCHLD is sent to the parent.
        child_proc.wait()

#----------------------------------------------------

stdout_log.close()
stderr_log.close()

sys.exit(0)
