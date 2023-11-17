"use client"

import React from 'react'
import Styles from "./page.module.css"
import Link from 'next/link'
import axios from 'axios'
import toast from 'react-hot-toast'
import { useRouter } from 'next/navigation'

function Login() {
  const router = useRouter();
  const [user, setUser] = React.useState({
    email: '',
    password: '',
  });
  


  const onLogin = async () => {
    try {
      const payload = await axios.post("http://localhost:8080/auth/login", user);
      if (payload.data.message === "EMAIL_IS_NOT_CONFIRMED") {
        toast.success("Logged in, but need to confirm email")

        let login_tmp_token = payload.headers['login_tmp_token']
        router.push("/email_verification?login_tmp_token=" + login_tmp_token)
        return
      }

      if (payload.data.error === true) {
        toast.error(payload.data.message)
      } else {
        let cookies = {
          username: payload.data.data.username,
          email: payload.data.data.email,
          accessToken: payload.headers['token'],
          refreshToken: payload.headers['refresh_token']
        }
        console.log(cookies)
        await axios.post('http://localhost:3000/api/set_auth_cookies', cookies);
        
        toast.success(payload.data.message)

        router.push("/")
      }
    } catch (error: any) {
      toast.error = (error)
    }
  }

  return (
    <div className={Styles.bodydiv}>
      <div className={Styles.loginContainer}>
        <h1>Login</h1>
        <div className={Styles.loginForm}>
          <input className={Styles.loginInput} type="text" placeholder="Email" name="email" value={user.email} onChange={(e) => setUser({ ...user, email: e.target.value })} />
          <input className={Styles.loginInput} type="text" placeholder="Password" name="password" value={user.password} onChange={(e) => setUser({ ...user, password: e.target.value })} />
          <p>Do not have an account?<Link href="/register" style={{ paddingLeft: "5px" }}>Register</Link></p>
          <button className={Styles.loginButton} type="submit" onClick={onLogin}>Login</button>
        </div>
      </div>
    </div>
  )
}

export default Login
