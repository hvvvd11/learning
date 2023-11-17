"use client";

import React from 'react'
import Styles from "./page.module.css"
import Link from 'next/link'
import toast from 'react-hot-toast';
import { useRouter } from 'next/navigation'
import axios from 'axios';

function Register() {
  let router = useRouter()
  const [user, setUser] = React.useState({
    username: '',
    password: '',
    email: '',
  });

  const onRegister = async () => {
    const payload = await axios.post("http://localhost:8080/auth/register", user);
    if (payload.data.error === true) {
      toast.error(payload.data.message);
    } else {
      toast.success(payload.data.message);

      let login_tmp_token = payload.headers['login_tmp_token']
      router.push("/email_verification?login_tmp_token=" + login_tmp_token)
    }
  };

  return (
    <div>
      <div className={Styles.bodydiv}>
        <div className={Styles.loginContainer}>
          <h1>Register</h1>
          <div className={Styles.loginForm}>
            <input className={Styles.loginInput} type="text" placeholder="Email" name="email" value={user.email} onChange={(e) => setUser({ ...user, email: e.target.value })} />
            <input className={Styles.loginInput} type="text" placeholder="Username" name="username" value={user.username} onChange={(e) => setUser({ ...user, username: e.target.value })} />
            <input className={Styles.loginInput} type="text" placeholder="Password" name="password" value={user.password} onChange={(e) => setUser({ ...user, password: e.target.value })} />
            <p>Already have an account? <Link href="/login" style={{ paddingLeft: "5px" }}>Login</Link></p>
            <button className={Styles.loginButton} type="submit" onClick={onRegister}>Register</button>
          </div>
        </div>
      </div>
    </div>
  )
}

export default Register
