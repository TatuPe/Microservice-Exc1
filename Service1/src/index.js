import { statfs } from 'fs';
import express from 'express'
import fetch from 'node-fetch'

//const express = require('express');
//const fetch = require('node-fetch');


const app = express();
//const port = process.env.PORT;
const port = "3000";

app.get("/log", async (req, res) => {
  console.log("GET log");
  res.status(200).send("OK");
});

app.get("/status", async (req, res) => {
  console.log("GET status");
  let free_space;

  try {
    const space = statfs('/', (err, stats) => {
      if (err) {
        throw err
      }
      free_space = (stats.bsize*stats.bfree)/1000000
    });

    const response = await fetch('http://service2:3001/status');
    const timestamp = new Date();

    let status = timestamp.toISOString() 
                  + ": uptime " + (process.uptime()/3600).toFixed(0) 
                  + " hours, free disk in root: " + free_space.toFixed(0) + " Mbytes"

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