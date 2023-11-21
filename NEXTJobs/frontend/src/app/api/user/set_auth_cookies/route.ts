import { NextResponse } from "next/server"

export async function POST(request: Request) {
    try {
    const reqBody = await request.json();

    const response = NextResponse.json(
      { message: "Login successful" },
      { status: 200 }
    );
   
    console.log(reqBody.token)
    console.log(reqBody.refreshToken)
    // set cookie
    response.cookies.set("refresh_token", reqBody.refreshToken, {httpOnly: true, maxAge: 60 * 60 * 24 * 30, sameSite:"strict"});
    response.cookies.set("token", reqBody.token, {httpOnly: true, maxAge:  5, sameSite:"strict"});
    response.cookies.set("username", reqBody.username, {httpOnly: true, maxAge: 60 * 60 * 24 * 30});
    response.cookies.set("email", reqBody.email, {httpOnly: true, maxAge: 60 * 60 * 24 * 30});
    
    return response;
  } catch (error: any) {
    return NextResponse.json({ message: error.message }, { status: 500 });
  }
}
