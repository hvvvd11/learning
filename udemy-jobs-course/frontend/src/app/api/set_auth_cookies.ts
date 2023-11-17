import { NextRequest, NextResponse } from "next/server"

export async function POST(request: Request) {
    try {
    const reqBody = await request.json();
    
    const response = NextResponse.json(
      { message: "Login successful" },
      { status: 200 }
    );

    // set cookie
    response.cookies.set("token", token, {httpOnly: true, maxAge: 60 * 60 * 24 * 1000});

    return response;
  } catch (error: any) {
    return NextResponse.json({ message: error.message }, { status: 500 });
  }

  const accessToken = req.body.accessToken || 'defaultAccessToken';
  const refreshToken = req.body.refreshToken || 'defaultRefreshToken';
  const email = req.body.email || 'defaultEmail';
  const username = req.body.username || 'defaultUsername';

  res.setHeader('Set-Cookie', [
    `token=${accessToken}; Path=/; Max-Age=3600; HttpOnly; SameSite=Strict`,
    `refresh_token=${refreshToken}; Path=/; Max-Age=86400; HttpOnly; SameSite=Strict`,
    `email=${email}; Path=/; Max-Age=3600; SameSite=Strict`,
    `username=${username}; Path=/; Max-Age=3600; SameSite=Strict`
  ]);

  res.status(200).json({ message: 'Tokens and user info set successfully' });
}
