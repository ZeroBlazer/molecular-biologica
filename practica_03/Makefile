CC = gcc
CFLAGS = -std=c++14 #-Wall
LIBS = -lstdc++

OBJS = main.o #parallel.o linear.o

EXECS=lab02
OBJS = main.o

all: ${EXECS}

run: lab02
	./lab02

lab02: ${OBJS}
	${CC} ${CFLAGS} -o lab02 ${OBJS} ${LIBS}

main.o: main.cpp
	${CC} ${CFLAGS} -c main.cpp ${LIBS}

# parallel.o: parallel.cpp parallel.h 
# 	${CC} ${CFLAGS} -c parallel.cpp ${LIBS}

# linear.o: linear.cpp linear.h 
# 	${CC} ${CFLAGS} -c linear.cpp ${LIBS}

clean:
	rm -f ${EXECS} ${OBJS}
	@echo "all cleaned up!"