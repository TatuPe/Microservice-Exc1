const express = require('express');
const fetch = require('node-fetch');

const app = express();
//const port = process.env.PORT;
const port = "3000";

app.get("/log", async (req, res) => {
  console.log("GET log");
  res.status(200).send("OK");
});

app.get("/status", async (req, res) => {
  console.log("GET status");

  try {
    const response = await fetch('http://service2:3001/status');
    res.status(200).send(await response.text());
  }
  catch (e) {
    console.log(e);
    res.status(500).send(e);
  }
});

app.listen(port, () => {
  console.log(`Server is running at http://localhost:${port}`);
});