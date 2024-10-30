import socket
import argparse

# Server function
def run_server(ip, port=65432):
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.bind((ip, port))
        s.listen()
        print(f"Server listening on {ip}:{port}...")
        
        conn, addr = s.accept()  # Accept connection
        print(f"Connected by {addr}")
        with conn:
            while True:
                data = conn.recv(1024)
                if not data:
                    print("Client disconnected.")
                    break
                print(f"Received: {data.decode('utf-8')}")
                conn.sendall(data)  # Echo the received message back to the client

# Client function
def run_client(server_ip, initial_message=None, port=65432):
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.connect((server_ip, port))
        print(f"Connected to server {server_ip}:{port}")
        
        # If an initial message is provided, send it
        if initial_message:
            s.sendall(initial_message.encode('utf-8'))
            data = s.recv(1024)
            print(f"Received from server: {data.decode('utf-8')}")
        
        # Continue sending messages in a loop
        try:
            while True:
                message = input("Enter message to send (type 'exit' to quit): ")
                if message.lower() == 'exit':
                    print("Exiting client.")
                    break
                s.sendall(message.encode('utf-8'))
                data = s.recv(1024)
                print(f"Received from server: {data.decode('utf-8')}")
        except KeyboardInterrupt:
            print("Client interrupted. Exiting...")

# Main function to handle arguments
def main():
    parser = argparse.ArgumentParser(description='TCP server-client script.')
    parser.add_argument('--server', type=str, help='Run as server. Provide the IP address to bind.')
    parser.add_argument('--client', type=str, help='Run as client. Provide the server IP to connect.')
    parser.add_argument('--message', type=str, help='Initial message to send when running as client.')

    args = parser.parse_args()

    if args.server:
        run_server(args.server)
    elif args.client:
        run_client(args.client, args.message)
    else:
        print("Please provide valid arguments. Use --server or --client.")

if __name__ == '__main__':
    main()
