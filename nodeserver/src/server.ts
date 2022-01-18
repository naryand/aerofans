import bcrypt from 'bcrypt'
import cookieParser from 'cookie-parser'
import dotenv from 'dotenv'
import express, { Request, Response, NextFunction } from 'express'
import fs from 'fs'
import helmet from 'helmet'
import https from 'https'
import morgan from 'morgan'
import { Pool } from 'pg'

//var privateKey  = fs.readFileSync('sslcert/selfsigned.key', 'utf8');
//var certificate = fs.readFileSync('sslcert/selfsigned.crt', 'utf8');
//var credentials = { key: privateKey, cert: certificate }

dotenv.config()
const app = express();
const DEFAULT_COST = 12
const PORT = process.env.PORT || 8443;
const pool = new Pool()

app.use(helmet());
app.use(morgan('tiny'));
app.use(express.json());
app.use(cookieParser())

//
// --------------------------- USERS ---------------------------
//

type User = {
  id: number;
  username: string;
  password: string;
  createdAt: Date;
}

// POST /register
// Takes in JSON encoded username, password
// On success, returns 200 OK with JSON encoded response
// On error, returns 500 Internal Server Error
const createUser = async (req: Request, res: Response) => {
  const { username, password } = req.body
  const hash = await bcrypt.hash(password, DEFAULT_COST)
  ;(
    async () => {
      await pool.query(
        'INSERT INTO users (username, password) VALUES ($1, $2)',
        [username, hash]
      )
      res.status(200).json({ status: true, message: 'registration successful' })
    }
  )().catch(_err => 
    setImmediate(() => {
      res.status(200).json({ status: false, message: 'username is taken' })
    })
  )
}

// POST /login
// Takes in JSON encoded username, password
// On success, returns 200 OK with JSON encoded response and cookie
// On error, returns 500 Internal Server Error
const loginUser = async (req: Request, res: Response) => {
  const { username, password } = req.body
  ;(
    async () => {
      const { rows } = await pool.query(
        'SELECT * FROM users WHERE username = $1',
        [username]
      )
      const result = await bcrypt.compare(password, rows[0].password)
      if(result) {
        const token = require('crypto').randomBytes(16).toString('base64')
        
        let expiry = new Date(Date.now())
        expiry.setHours(expiry.getHours() + 1)
        res.cookie('token', token, { expires: expiry })
        
        ;(
          async () => {
            await pool.query(
              'INSERT INTO tokens (id, user_id, expires_at) VALUES ($1, $2, $3)',
              [token, rows[0].id, expiry]
            )
            res.status(200).json({ status: true, message: "login successful" })
          }
        )().catch(_err =>
          setImmediate(() => {
            res.status(500).json({ status: false, message: "internal server error" })
          })
        )

      } else {
        res.status(200).json({ status: false, message: "incorrect login info" })
      }
    }
  )().catch(_err => 
    setImmediate(() => {
      res.status(200).json({ status: false, message: "username doesn't exist" })
    })
  )
}

// Implements user authentication
// Takes token from cookie and checks against database
const authenticate = async (req: Request, res: Response, next: NextFunction) => {
  if(req.cookies) {
    const cookie = req.cookies['token']
    if(cookie) { 
      ;(
        async () => {
          const { rows } = await pool.query(
            'SELECT * FROM tokens WHERE id = $1',
            [cookie]
          )
          const expiry = new Date(rows[0].expires_at)
          const now = new Date(Date.now())
          if(expiry > now) {
            res.locals.userId = rows[0].user_id
            next()
          } else {
            res.status(401).end()
          }
        }
      )().catch(_err => 
        setImmediate(() => {
          res.status(401).end()
        })
      )
    } else {
      res.status(400).end()
    }
  } else {
    res.status(400).end()
  }
}

// User routes
app.post('/register', createUser)
app.post('/login', loginUser)

//
// --------------------------- POSTS ---------------------------
//

type Post = {
  id: number;
  author: number;
  username: string;
  text: string;
  createdAt: Date;
}

// POST /post
// Takes in JSON encoded text and user auth
// On success, returns 200 OK with JSON encoded Post
// On error, returns 500 Internal Server Error
const createPost = async (req: Request, res: Response) => {
  const { text } = req.body
  ;(
    async () => {
      const { rows } = await pool.query(
        'WITH updated AS (INSERT INTO posts (text, author) VALUES ($1, $2) RETURNING *) \
        SELECT updated.*, users.username FROM updated INNER JOIN users ON updated.author = users.id',
        [text, res.locals.userId]
      )
      res.status(200).send(rows[0])
    }
  )().catch(_err => 
    setImmediate(() => {
      res.status(500).end()
    })
  )
}

// GET /post/all
// On success, returns 200 OK with JSON encoded Posts
// On error, returns 500 Internal Server Error
const readAllPosts = async (_req: Request, res: Response) => {
  ;(
    async () => {
      const { rows } = await pool.query(
        'SELECT posts.*, users.username FROM posts INNER JOIN users ON posts.author = users.id',
      )
      res.status(200).send(rows)
    }
  )().catch(_err => 
    setImmediate(() => {
      res.status(500).end()
    })
  )
}

// GET /post/:postId
// On success, returns 200 OK with JSON encoded Post
// If postId does not exist, returns 404 Not Found
const readPost = async (req: Request, res: Response) => {
  const postId = parseInt(req.params.postId)
  ;(
    async () => {
      const { rows } = await pool.query(
        'SELECT posts.*, users.username \
        FROM posts INNER JOIN users ON posts.author = users.id \
        WHERE posts.id = $1',
        [postId]
      )
      if(rows[0]) {
        res.status(200).send(rows[0])
      } else {
        res.status(404).end()
      }
    }
  )().catch(_err => 
    setImmediate(() => {
      res.status(404).end()
    })
  )
}

// PATCH /post/:postId
// Takes in JSON encoded text and user auth
// On success, updates and returns 200 OK with JSON encoded Post
// If postId does not exist, returns 404 Not Found
const updatePost = async (req: Request, res: Response) => {
  const postId = parseInt(req.params.postId)
  const { text } = req.body
  ;(
    async () => {
      const { rows } = await pool.query(
        'SELECT posts.*, users.username \
        FROM posts INNER JOIN users ON posts.author = users.id \
        WHERE posts.id = $1',
        [postId]
      )
      if(rows[0]) {
        if(rows[0].author != res.locals.userId) {
          res.status(401).end()
        } else {
          ;(
            async () => {
              const { rows } = await pool.query(
                'WITH updated AS (UPDATE posts SET text = $1 WHERE id = $2 RETURNING *) \
                SELECT updated.*, users.username FROM updated INNER JOIN users ON updated.author = users.id \
                WHERE updated.id = $2',
                [text, postId]
              )
              res.status(200).send(rows[0])
            }
          )().catch(_err => 
            setImmediate(() => {
              res.status(404).end()
            })
          )
        }
      } else {
        res.status(404).end()
      }
    }
  )().catch(_err => 
    setImmediate(() => {
      res.status(404).end()
    })
  )
}

// DELETE /post/:postId
// Takes in user auth
// On success, deletes and returns 200 OK
// If postId does not exist, returns 404 Not Found
const deletePost = async (req: Request, res: Response) => {
  const postId = parseInt(req.params.postId)
  ;(
    async () => {
      const { rows } = await pool.query(
        'SELECT posts.*, users.username \
        FROM posts INNER JOIN users ON posts.author = users.id \
        WHERE posts.id = $1',
        [postId]
      )
      if(rows[0]) {
        if(rows[0].author != res.locals.userId) {
          res.status(401).end()
        } else {
          ;(
            async () => {
              const { rows } = await pool.query(
                'DELETE FROM posts WHERE id = $1',
                [postId]
              )
              res.status(200).end()
            }
          )().catch(_err => 
            setImmediate(() => {
              res.status(404).end()
            })
          )
        }
      } else {
        res.status(404).end()
      }
    }
  )().catch(_err => 
    setImmediate(() => {
      res.status(404).end()
    })
  )
}

// Post routes
app.post('/post', authenticate, createPost)
app.get('/post/all', readAllPosts)
app.get('/post/:postId', readPost)
app.patch('/post/:postId', authenticate, updatePost)
app.delete('/post/:postId', authenticate, deletePost)

//
// --------------------------- REPLIES ---------------------------
//

type Reply = {
  id: number;
  postId: number;
  author: number;
  username: string;
  text: string;
  createdAt: Date;
}

// POST /post/:postId/reply
// Takes in JSON encoded text and user auth
// On success, returns 200 OK with JSON encoded Reply
// If postId does not exist, returns 404 Not Found
const createReply = async (req: Request, res: Response) => {
  const postId = parseInt(req.params.postId)
  const { text } = req.body
  ;(
    async () => {
      const { rows } = await pool.query(
        'WITH updated AS \
        (INSERT INTO replies (text, post_id, author) VALUES ($1, $2, $3) RETURNING *) \
        SELECT updated.*, users.username FROM updated INNER JOIN users ON updated.author = users.id',
        [text, postId, res.locals.userId]
      )
      res.status(200).send(rows[0])
    }
  )().catch(_err => 
    setImmediate(() => {
      res.status(404).end()
    })
  )
}

// GET /post/:postId/reply/all
// On success, returns 200 OK with JSON encoded Replies
// If postId does not exist, returns 404 Not Found
const readAllReplies = async (req: Request, res: Response) => {
  const postId = parseInt(req.params.postId)
  ;(
    async () => {
      const { rows } = await pool.query(
        'SELECT replies.*, users.username FROM replies \
        INNER JOIN users ON replies.author = users.id \
        WHERE replies.post_id = $1',
        [postId]
      )
      res.status(200).send(rows)
    }
  )().catch(_err => 
    setImmediate(() => {
      res.status(404).end()
    })
  )
}

// GET /post/:postId/reply/:replyId
// On success, returns 200 OK with JSON encoded Reply
// If postId, replyId does not exist, returns 404 Not Found
const readReply = async (req: Request, res: Response) => {
  const replyId = parseInt(req.params.replyId)
  const postId = parseInt(req.params.postId)
  ;(
    async () => {
      const { rows } = await pool.query(
        'SELECT replies.*, users.username FROM replies \
        INNER JOIN users ON replies.author = users.id \
        WHERE replies.id = $1 AND replies.post_id = $2',
        [replyId, postId]
      )
      if(rows[0]) {
        res.status(200).send(rows[0])
      } else {
        res.status(404).end()
      }
    }
  )().catch(_err => 
    setImmediate(() => {
      res.status(404).end()
    })
  )
}

// PATCH /post/:postId/reply/:replyId
// Takes in JSON encoded text and user auth
// On success, updates and returns 200 OK with JSON encoded Reply
// If postId, replyId does not exist, returns 404 Not Found
const updateReply = async (req: Request, res: Response) => {
  const { text } = req.body
  const replyId = parseInt(req.params.replyId)
  const postId = parseInt(req.params.postId)
  ;(
    async () => {
      const { rows } = await pool.query(
        'SELECT replies.*, users.username FROM replies \
        INNER JOIN users ON replies.author = users.id \
        WHERE replies.id = $1 AND replies.post_id = $2',
        [replyId, postId]
      )
      if(rows[0]) {
        if(rows[0].author != res.locals.userId) {
          res.status(401).end()
        } else {
          ;(
            async () => {
              const { rows } = await pool.query(
                'WITH updated AS (UPDATE replies SET text = $1 WHERE id = $2 AND post_id = $3 RETURNING *) \
                SELECT updated.*, users.username FROM updated INNER JOIN users ON updated.author = users.id \
                WHERE updated.id = $2 AND updated.post_id = $3',
                [text, replyId, postId]
              )
              res.status(200).send(rows[0])
            }
          )().catch(_err => 
            setImmediate(() => {
              res.status(404).end()
            })
          )
        }
      } else {
        res.status(404).end()
      }
    }
  )().catch(_err => 
    setImmediate(() => {
      res.status(404).end()
    })
  )
}

// DELETE /post/:postId/reply/:replyId
// Takes in user auth
// On success, deletes and returns 200 OK
// If postId, replyId does not exist, returns 404 Not Found
const deleteReply = async (req: Request, res: Response) => {
  const postId = parseInt(req.params.postId)
  const replyId = parseInt(req.params.replyId)
  ;(
    async () => {
      const { rows } = await pool.query(
        'SELECT replies.*, users.username FROM replies \
        INNER JOIN users ON replies.author = users.id \
        WHERE replies.id = $1 AND replies.post_id = $2',
        [replyId, postId]
      )
      if(rows[0]) {
        if(rows[0].author != res.locals.userId) {
          res.status(401).end()
        } else {
          ;(
            async () => {
              const { rows } = await pool.query(
                'DELETE FROM replies WHERE id = $1 AND post_id = $2',
                [replyId, postId]
              )
              res.status(200).end()
            }
          )().catch(_err => 
            setImmediate(() => {
              res.status(404).end()
            })
          )
        }
      } else {
        res.status(404).end()
      }
    }
  )().catch(_err => 
    setImmediate(() => {
      res.status(404).end()
    })
  )
}

// Reply routes
app.post('/post/:postId/reply', authenticate, createReply)
app.get('/post/:postId/reply/all', readAllReplies)
app.get('/post/:postId/reply/:replyId', readReply)
app.patch('/post/:postId/reply/:replyId', authenticate, updateReply)
app.delete('/post/:postId/reply/:replyId', authenticate, deleteReply)


// Start server
//const httpsServer = https.createServer(credentials, app)
app.listen(PORT, () => {
  console.log(`[server]: Listening on port: ${PORT}`);
});
