package main

import (
	"flag"
	"fmt"
	"log"
	"net"
	"os"
	"os/signal"
	"strconv"
	"sync"
	"syscall"
)

func startServer(port int, wg *sync.WaitGroup) {
	defer wg.Done()
	listener, err := net.Listen("tcp", fmt.Sprintf(":%d", port))
	if err != nil {
		log.Printf("Failed to start server on port %d: %v", port, err)
		return
	}
	defer listener.Close()

	log.Printf("Server listening on port %d", port)

	for {
		conn, err := listener.Accept()
		if err != nil {
			log.Printf("Error accepting connection on port %d: %v", port, err)
			return
		}
		go handleConnection(conn, port)
	}
}

func handleConnection(conn net.Conn, port int) {
	defer conn.Close()
	log.Printf("New connection on port %d from %s", port, conn.RemoteAddr().String())
}

func main() {
	// Define usage
	flag.Usage = func() {
		fmt.Fprintf(os.Stderr, "Usage: %s port1 port2 port3 ...\n", os.Args[0])
		fmt.Fprintf(os.Stderr, "Example: %s 8080 8081 8082\n", os.Args[0])
		flag.PrintDefaults()
	}

	flag.Parse()

	// Check if ports were provided
	if flag.NArg() == 0 {
		flag.Usage()
		os.Exit(1)
	}

	var ports []int
	// Parse port arguments
	for _, arg := range flag.Args() {
		port, err := strconv.Atoi(arg)
		if err != nil {
			log.Fatalf("Invalid port number: %s", arg)
		}
		if port < 1 || port > 65535 {
			log.Fatalf("Port number %d is out of range (1-65535)", port)
		}
		ports = append(ports, port)
	}

	var wg sync.WaitGroup

	// Start servers on all ports
	for _, port := range ports {
		wg.Add(1)
		go startServer(port, &wg)
	}

	// Handle graceful shutdown
	sigChan := make(chan os.Signal, 1)
	signal.Notify(sigChan, syscall.SIGINT, syscall.SIGTERM)
	<-sigChan

	log.Println("Shutting down servers...")
}
