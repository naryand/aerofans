const express = require('express');
const helmet = require('helmet');
const morgan = require('morgan');
const app = express();

const PORT = process.env.PORT || 9000;

app.use(helmet());
app.use(morgan('tiny'));

app.get('/', (req, res) => {
  res.send('Welcome to aeroFans 👻');
});

app.get('/secret', (req, res) => {
  res.send('Secret page 👻');
});

app.listen(PORT, () => {
  console.log(`[server]: Listening on port: ${PORT}`);
});
