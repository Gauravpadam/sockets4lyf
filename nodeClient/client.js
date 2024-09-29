const { log } = require('console');
const readline = require('node:readline/promises');
const io = require("socket.io-client");

async function main(){
    const rl = readline.createInterface({input: process.stdin, output: process.stdout});

    // Await the user's input
    const text = await rl.question("Enter message: ");

    const socket = io('ws://127.0.0.1:3000');

    // Ensure socket is connected before emitting the event
    socket.on('connect', () => {
        console.log('Connected to server.');

        // Emit the event after connection is established
    
        socket.emit("message", text, (response) => {
            console.log(response.status); // ok
        });
    
    socket.on("server_message", async (body) => {
        console.log(`Them: ${body}`);

        socket.emit("message", await rl.question("You: "));

    })
    });

    socket.on('connect_error', (err) => {
        console.error('Connection error:', err.message);
        rl.close();
    });

    return 1;
}

main();
