CC = gcc
CFLAGS = -Wall -O0 -s
SRCS = git-helper.c
TARGET = git-helper
CONF = git-helper.conf
DESTDIR = /usr/bin/
CONFDIR = ~/.config/git-helper/


all:
	$(CC) $(SRCS) $(CFLAGS) -o $(TARGET)
static:
	$(CC) $(SRCS) $(CFLAGS) -static -o $(TARGET)
install:
	@cp -p $(TARGET) $(DESTDIR)$(TARGET)
	@mkdir -p $(CONFDIR)
	@cp -p $(CONF) $(CONFDIR)
uninstall:
	@rm -rf $(DESTDIR)$(TARGET)
	@rm -rf $(CONFDIR)
