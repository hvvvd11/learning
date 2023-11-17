import { cookies } from 'next/headers'

export default function Home() {
  let token = String(cookies().get("token"))
  let email = String(cookies().get("email"))
  let username = String(cookies().get("username"))
  let refresh_token = String(cookies().get("refresh_token"))
  return (
    <>
      <h1>Jobs project</h1>
      <p>{token}</p>
      <p>{email}</p>
      <p>{username}</p>
      <p>{refresh_token}</p>
    </>
  )
}
