#include <fcntl.h>
#include <sys/socket.h>
#include <resolv.h>
#include <netdb.h>
#include <netinet/in.h>
#include <netinet/ip_icmp.h>
#include <stdbool.h>
#include <unistd.h>
#include <string.h>

struct packet {
    struct icmphdr hdr;
    char msg[64 - sizeof(struct icmphdr)];
};

// checksum - standard 1s complement checksum
unsigned short checksum(void *b, int len) {
    unsigned short *buf = b;
    unsigned int sum = 0;
    unsigned short result;
    for (sum = 0; len > 1; len -= 2)
        sum += *buf++;
    if (len == 1)
        sum += *(unsigned char *) buf;
    sum = (sum >> 16) + (sum & 0xFFFF);
    sum += (sum >> 16);
    result = ~sum;
    return result;
}

bool ping(char *adress) {
    struct sockaddr_in r_addr;
    struct sockaddr_in addr_ping, *addr;
    struct hostent *hname = gethostbyname(adress);
    bzero(&addr_ping, sizeof(addr_ping));
    addr_ping.sin_family = hname->h_addrtype;
    addr_ping.sin_port = 0;
    addr_ping.sin_addr.s_addr = *(long *) hname->h_addr;
    addr = &addr_ping;

    int sd = socket(AF_INET, SOCK_DGRAM, IPPROTO_ICMP);
//    int sd = socket(PF_INET, SOCK_RAW, IPPROTO_ICMP);
    if (sd < 0) {
        perror("socket");
        return false;
    }
    if (fcntl(sd, F_SETFL, O_NONBLOCK) != 0) {
        perror("fcntl");
        return false;
    }
    const int ttl_val = 64;
    if (setsockopt(sd, SOL_IP, IP_TTL, &ttl_val, sizeof(ttl_val)) != 0) {
        perror("setsockopt");
        return false;
    }

//    int cnt = 1;
    for (int loop = 0; loop < 10; loop++) {
        int len = sizeof(r_addr);
        printf("before recvfrom\n");
        struct packet pckt;
        if (recvfrom(sd, &pckt, sizeof(pckt), 0, (struct sockaddr *) &r_addr, &len) > 0) {
            return true;
        }
        printf("after recvfrom\n");

        bzero(&pckt, sizeof(pckt));
        pckt.hdr.type = ICMP_ECHO;
//        int i = 0;
        for (int i = 0; i < sizeof(pckt.msg) - 1; i++)
            pckt.msg[i] = i + '0';
//        pckt.msg[i] = 0;
//        pckt.hdr.un.echo.sequence = cnt++;
        pckt.hdr.checksum = checksum(&pckt, sizeof(pckt));
        printf("checksum = %d\n", pckt.hdr.checksum);
        if (sendto(sd, &pckt, sizeof(pckt), 0, (struct sockaddr *) addr, sizeof(*addr)) <= 0)
            perror("sendto");
        usleep(300 * 1000);
    }
    return false;
}

int main() {
    if (ping("www.baidu.com") == true) {
        printf("Ping is OK. \n");
    } else {
        printf("Ping is not OK. \n");
    }
    return 0;
}
