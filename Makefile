CC = gcc
CFLAGS = -Wall -O0 -s
SRCS = git-helper.c
TARGET = git-helper
DESTDIR = /usr/bin/


all:
	$(CC) $(SRCS) $(CFLAGS) -o $(TARGET)
static:
	$(CC) $(SRCS) $(CFLAGS) -static -o $(TARGET)
install:
	@cp -p $(TARGET) $(DESTDIR)$(TARGET)
uninstall:
	@rm -rf $(DESTDIR)$(TARGET)
