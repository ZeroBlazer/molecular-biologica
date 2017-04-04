CC = g++
CFLAGS = -std=c++14 #-Wall
LIBS = -lstdc++

OBJS = main.o amino-lib.o

run: chain
	@echo "\nRUNNING PROGRAM!"
	./chain

chain: ${OBJS}
	${CC} ${CFLAGS} -o chain ${OBJS} ${LIBS}

main.o: main.cpp
	${CC} ${CFLAGS} -c main.cpp ${LIBS}

amino-lib.o: amino-lib.cpp amino-lib.h
	${CC} ${CFLAGS} -c amino-lib.cpp ${LIBS}

clean:
	rm -f chain ${OBJS}
	@echo "all cleaned up!"