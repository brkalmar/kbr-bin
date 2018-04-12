CXXFLAGS += -std=c++11 -Wall -Wextra -Wpedantic -march=native -pipe

DEBUG ?= 0
ifeq ($(DEBUG), 1)
	CFLAGS += -g -Og
else
	CFLAGS += -O3
	CPPFLAGS += -DNDEBUG
endif

BIN = ord

$(BIN) : main.o
	$(CXX) $(LDFLAGS) -o $@ $^

.PHONY : clean
clean :
	$(RM) $(BIN) *.o
