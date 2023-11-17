import { NextResponse } from "next/server"

export async function POST(request: Request) {
    try {
    const reqBody = await request.json();

    console.log(reqBody) 

    const response = NextResponse.json(
      { message: "Login successful" },
      { status: 200 }
    );

    // set cookie
    response.cookies.set("refresh_token", reqBody.refreshToken, {httpOnly: true, maxAge: 60 * 60 * 24 * 30, sameSite:"strict"});
    response.cookies.set("token", reqBody.token, {httpOnly: true, maxAge: 60 * 60 * 2, sameSite:"strict"});
    response.cookies.set("username", reqBody.username, {httpOnly: true, maxAge: 60 * 60 * 2});
    response.cookies.set("email", reqBody.email, {httpOnly: true, maxAge: 60 * 60 * 2});

    return response;
  } catch (error: any) {
    return NextResponse.json({ message: error.message }, { status: 500 });
  }
}
