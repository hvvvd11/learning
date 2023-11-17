import { cookies } from 'next/headers'

export default function Home() {
  let token = cookies().get("token")?.value
  let email = cookies().get("email")?.value
  let username = cookies().get("username")?.value
  let refresh_token = cookies().get("refresh_token")?.value
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
