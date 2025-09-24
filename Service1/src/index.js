import { readFileSync, writeFileSync, statfsSync } from 'fs';
import express from 'express'
import fetch from 'node-fetch'

const VSTORAGE = "/vstorage"; 

const app = express();
//const port = process.env.PORT;
const port = "3000";

app.get("/log", async (req, res) => {
  console.log("GET log");
  const response = await fetch('http://storage:3002/log');
  res.type('text');
  res.status(200).send((await response.text()));
});

app.get("/status", async (req, res) => {
  console.log("GET status");
  let free_space;

  // Get free space
  try {
    let stats = statfsSync('/');
    free_space = ((stats.bsize*stats.bfree)/1000000).toFixed(0)

    // Create status string
    const timestamp = new Date();
    let status = timestamp.toISOString() 
                  + ": uptime " + (process.uptime()/3600).toFixed(0) 
                  + " hours, free disk in root: " + free_space + " Mbytes"

    // Write status to file
    const file = readFileSync(VSTORAGE, function(err) {
    if(err) {
        console.log(err);
    }}).toString();
    
    writeFileSync(VSTORAGE, file + status + "\n", function(err) {
    if(err) {
        console.log(err);
    }
    else{
      console.log("vStorage write success!");
    }
    });

    // POST Status to storage
    const post = await fetch('http://storage:3002/log', {method: 'POST', body: status});
    console.log("Storage: " + await post.text());

    // Get status from service 2
    const response = await fetch('http://service2:3001/status');

    // Create response
    res.type('text');
    res.status(200).send(status + "\n" + await response.text());
  }
  catch (e) {
    console.log(e);
    res.status(500).send(e);
  }
});

app.listen(port, () => {
  console.log(`Server is running at http://localhost:${port}`);
});