#include <linux/input.h>
#include <string.h>
#include <fcntl.h>
#include <stdio.h>
#include <signal.h>
#include <time.h>
#include <stdlib.h>
#include <unistd.h>

#define BITS_PER_LONG (sizeof(long) * 8)
#define NBITS(x) ((((x)-1)/BITS_PER_LONG)+1)
#define OFF(x)  ((x)%BITS_PER_LONG)
#define BIT(x)  (1UL<<OFF(x))
#define LONG(x) ((x)/BITS_PER_LONG)
#define test_bit(bit, array)	((array[LONG(bit)] >> OFF(bit)) & 1)

char *absval[6] = { "Value", "Min", "Max", "Fuzz", "Flat", "Resolution"};

void get_input_details(int fd) {
	unsigned short id[4];
        unsigned long bit[EV_MAX][NBITS(KEY_MAX)];
        char name[256] = "Unknown";
        int abs[6] = {0};

	// get exclusive access
	ioctl(fd, EVIOCGRAB, 1);

	// get input device name
        ioctl(fd, EVIOCGNAME(sizeof(name)), name);
        printf("Input device name: \"%s\"\n", name);

	// get supported events
        memset(bit, 0, sizeof(bit));
        ioctl(fd, EVIOCGBIT(0, EV_MAX), bit[0]);
        printf("Supported events:\n");

        int i,j,k;
        for (i = 0; i < EV_MAX; i++) {
                if (test_bit(i, bit[0])) {
                        printf("  Event type 0x%x\n", i);
                        if (!i) continue;
                        ioctl(fd, EVIOCGBIT(i, KEY_MAX), bit[i]);
                        for (j = 0; j < KEY_MAX; j++) {
                                if (test_bit(j, bit[i])) {
                                        printf("    Event code 0x%x\n", j);
                                        if (i == EV_ABS) {
                                                ioctl(fd, EVIOCGABS(j), abs);
                                                for (k = 0; k < 5; k++) {
                                                        if ((k < 3) || abs[k]){
                                                                printf("     %s %d\n", absval[k], abs[k]);
                                                        }
                                                }

                                        }
                                }
                        }
		}
	}
}

