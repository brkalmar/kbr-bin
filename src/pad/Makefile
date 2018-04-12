CFLAGS += -std=c11 -Wall -Wextra -Wpedantic -march=native -pipe

DEBUG ?= 0
ifeq ($(DEBUG), 1)
	CFLAGS += -g -Og
else
	CFLAGS += -O3
	CPPFLAGS += -DNDEBUG
endif

BIN = pad

$(BIN) : main.o
	$(CC) $(LDFLAGS) -o $@ $^

.PHONY : clean
clean :
	$(RM) $(BIN) *.o
