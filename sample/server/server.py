import socket
import os

def safe_write(raw_filename):
	i = 1
	while True:
		filename = '{}-{:02.0f}.txt'.format(raw_filename, i)
		if os.path.isfile(filename):
			i += 1
			continue
		return open(filename, 'wb')

def save_file(connection, filename):
	with safe_write(filename) as f:
		while True:
			data = connection.recv(1024)
			if not data:
				break
			f.write(data)
		print("File {} received and saved successfully.".format(filename))

def main():
	host = '0.0.0.0' # 0.0.0.0 for any IP
	port = 4242

	server_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
	server_socket.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
	try:
		server_socket.bind((host, port))
		server_socket.listen(1) # listen for one incoming connection

		print("Server listening on {}:{}".format(host, port))

		while True:
			client_socket, addr = server_socket.accept()
			print("Connection from:", addr)

			save_file(client_socket, f"data_{addr[0]}")

			client_socket.close()
	except KeyboardInterrupt:
		server_socket.close()

if __name__ == "__main__":
	main()
