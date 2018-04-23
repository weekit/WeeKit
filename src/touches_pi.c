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

int fd;

int openTouchScreen()
{
        if ((fd = open("/dev/input/event1", O_RDONLY)) < 0) {
                return 1;
        }
	return 0;
}

char *absval[6] = { "Value", "Min", "Max", "Fuzz", "Flat", "Resolution"};

void getTouchScreenDetails(int *xmin,int *xmax,int *ymin,int *ymax)
{
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
                                                                if (j == 0){
                                                                        if (k == 1) *xmin =  abs[k];
                                                                        if (k == 2) *xmax =  abs[k];
                                                                }
                                                                if (j == 1){
                                                                        if (k == 1) *ymin =  abs[k];
                                                                        if (k == 2) *ymax =  abs[k];
                                                                }
                                                        }
                                                }

                                        }
                                }
                        }
		}
	}
}

typedef void (*WKEventHandler)(short, short, int);
extern WKEventHandler wkEventHandler;

void getTouchSample(int *rawX, int *rawY, int *rawPressure) {
        /* the events (up to 64 at once) */
        struct input_event ev[64];

        /* how many bytes were read */
	size_t rb = read(fd, ev, sizeof(struct input_event)*64);

	int i;
        for (i = 0; i <  (rb / sizeof(struct input_event)); i++){
		unsigned short t = ev[i].type;
		unsigned short c = ev[i].code;
		int v = ev[i].value;
		wkEventHandler(t, c, v);
	}
}

int start_touches() {
	if (openTouchScreen() == 1) {
		perror("error opening touch screen");
		return -1;
	}
	int xmax, xmin, ymax, ymin;
	getTouchScreenDetails(&xmin,&xmax,&ymin,&ymax);
	return 0;
}

void handle_touches() {
		int rawX, rawY, rawPressure;
                getTouchSample(&rawX, &rawY, &rawPressure);
}
