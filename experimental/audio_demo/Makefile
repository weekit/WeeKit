
BIN=audio_demo.bin

OBJS=audio.o sinewave.o main.o ilclient.o ilcore.o

CFLAGS+= \
	-g \
	-DOMX_SKIP64BIT \
	-DUSE_VCHIQ_ARM \
	-std=gnu99 \
	-Wall \
	-Wno-deprecated-declarations \
	-Wno-psabi 

LDFLAGS+=-L/opt/vc/lib/ \
	-lbcm_host \
	-lopenmaxil \
	-lvcos \
	-lpthread \
	-lm

INCLUDES+= -I/opt/vc/include/ 

all: $(BIN) $(LIB)

%.o: %.c
	$(CC) $(CFLAGS) $(INCLUDES) -c $< -o $@ 

%.bin: $(OBJS)
	$(CC) -o $@ $(OBJS) $(LDFLAGS) 

clean:
	for i in $(OBJS); do (if test -e "$$i"; then ( rm $$i ); fi ); done
	@rm -f $(BIN) $(LIB)
