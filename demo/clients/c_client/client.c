#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <netinet/in.h>
#include <sys/socket.h>
#include <unistd.h>
#include <arpa/inet.h> // Added for inet_addr
#include <openssl/ssl.h>
#include <openssl/err.h>

#define PORT 3001

int main(int argc, char *argv[]) {
    // Initialize OpenSSL
    SSL_load_error_strings();
    OpenSSL_add_ssl_algorithms();

    // Create SSL context
    const SSL_METHOD *method = TLS_client_method();
    SSL_CTX *ctx = SSL_CTX_new(method);
    if (!ctx) {
        ERR_print_errors_fp(stderr);
        exit(EXIT_FAILURE);
    }

    // Create socket
    int sock = socket(AF_INET, SOCK_STREAM, 0);
    if (sock < 0) {
        perror("socket failed");
        exit(EXIT_FAILURE);
    }

    // Get last octet from command line or default to 6
    int last_octet = 6;  // Added default value
    //printf("argc = %d\n", argc);  // Added debug output for argument count
    if (argc > 1) {  // Added argument checking
        last_octet = atoi(argv[1]);  // Added conversion of argument to integer
        if (last_octet < 0 || last_octet > 255) {  // Added validation
            printf("Invalid octet value. Must be between 0 and 255\n");
            exit(EXIT_FAILURE);
        }
    }

    // Connect to server
    struct sockaddr_in server_addr;
    server_addr.sin_family = AF_INET;
    server_addr.sin_port = htons(PORT);
    //server_addr.sin_addr.s_addr = inet_addr("127.0.2.6");
    char ip_addr[16];  // Added buffer for IP string
    snprintf(ip_addr, sizeof(ip_addr), "127.0.2.%d", last_octet);  // Added dynamic IP construction
    server_addr.sin_addr.s_addr = inet_addr(ip_addr);  // Modified to use dynamic IP

    printf("Connecting to server at %s\n", ip_addr);

    if (connect(sock, (struct sockaddr *)&server_addr, sizeof(server_addr)) < 0) {
        perror("connect failed");
        exit(EXIT_FAILURE);
    }

    printf("Connected to server at %s\n", ip_addr);  // Modified to show actual IP used

    // Set up SSL
    SSL *ssl = SSL_new(ctx);
    if (!ssl) {
        ERR_print_errors_fp(stderr);
        exit(EXIT_FAILURE);
    }

    SSL_set_fd(ssl, sock);

    // Attempt TLS handshake
    int err = SSL_connect(ssl);
    if (err <= 0) {
        printf("SSL handshake failed\n");
        ERR_print_errors_fp(stderr);
        SSL_free(ssl);
        close(sock);
        SSL_CTX_free(ctx);
        exit(EXIT_FAILURE);
    }

    printf("SSL handshake succeeded\n");

    // Cleanup
    SSL_free(ssl);
    close(sock);
    SSL_CTX_free(ctx);
    return 0;
}
