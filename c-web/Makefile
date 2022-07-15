PROG ?= c-web
ROOT ?= $(realpath $(CURDIR)/libs)
DEFS ?= -DMG_ENABLE_LINES=1 -DMG_ENABLE_IPV6=1 -DMG_ENABLE_SSI=1
CFLAGS ?= -Ilibs -W -Wall $(DEFS) $(EXTRA)

# Add static linking to test on a non-Mac platform since Mac is stupid and doesn't allow it
# https://www.systutorials.com/how-to-statically-link-c-and-c-programs-on-linux-with-gcc/
$(PROG): main.c Makefile
	$(CC) libs/mongoose.c main.c $(CFLAGS) -o $@