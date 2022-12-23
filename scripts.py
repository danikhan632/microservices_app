import os
import sys
def main():
    if len(sys.argv) == 1:
        str = ("Usage\r\n\r\n"
   "scripts.py docker \""
   "your argument\"     "
   "  runs with cargo \""
   "your argument\"\r\n");
        print(str)
    
    elif len(sys.argv) == 2 and sys.argv[1] == "build":
        docker_build()
    elif len(sys.argv) == 2 and sys.argv[1] == "run":
        docker_run()

    
def docker_build(build_com="build",run_com="run"):
    
    os.system("sudo docker container rm rs_container")
    os.system("sudo docker build -t rs_builder . ")
    os.system("sudo docker create --name rs_container rs_builder ")
    os.system("rm -rf ./target")
    os.system("sudo docker cp rs_container:/app/target ./")
    os.system("sudo docker container rm rs_container")
    os.system("sudo chmod -R 777 ./target")

def docker_run(build_com="build",run_com="run"):
    os.system("sudo docker container rm rs_container")
    os.system("sudo docker build -t rs_builder .")
    os.system("sudo docker run --name rs_container rs_builder ")
    os.system("sudo docker container rm rs_container")

    
if __name__ == "__main__":
    main()