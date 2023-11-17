"use client"

import axios from 'axios';
import React, { useState, ChangeEvent} from 'react';
import toast from 'react-hot-toast';
import { useSearchParams } from 'next/navigation'
import { useRouter } from 'next/navigation'
import Link from 'next/link'

const VerifyEmail: React.FC = () => {
  const router = useRouter();
  const [code, setCode] = useState<string[]>(Array(6).fill(''));
  const searchParams = useSearchParams()
 
  const sendVerificationCode = async () => {
    try {
      let verification_code = code.join('');
      let requestPayload = {
        login_tmp_token: searchParams.get('login_tmp_token'),
        verification_code: Number(verification_code),
      }

      const payload = await axios.post('http://localhost:8080/user/confirm_email', requestPayload);

      if (payload.data.error === true) {
        toast.error(payload.data.message)
      } else {
        toast.success(payload.data.message)

        const token = payload.headers['token']

        // cookiesSet("username", payload.data.data.username)
        // cookiesSet("email", payload.data.data.email)
        // cookiesSet("token", token)

        router.push("/")
      }
    } catch (error: any) {
      toast.error = (error)
    }
  }

  // behaviour of an input element
  const handleInput = (index: number, event: ChangeEvent<HTMLInputElement>) => {
  const value = event.target.value;
 
    // Create a new code array to ensure immutability
    const newCode = [...code];
    
    // Check if the input value is a valid digit or is empty
    if (/^[0-9]$/.test(value) || value === '') {
      newCode[index] = value;
      setCode(newCode);
      
      // Focus on the next input if current input has a value and isn't the last one
      if (value && index !== 5) {
        const nextInput = document.getElementById(`code-${index + 1}`) as HTMLInputElement;
        nextInput?.focus();
      }
    }
  };
  return (
    <div style={{ display: 'flex', height: '80vh', alignItems: 'center', justifyContent: 'center' }}>
      <div className="container" style={{ width: '400px', height: '400px', display: 'flex', flexDirection: 'column', alignItems: 'center', justifyContent: 'center', textAlign: 'center' }}>
        <h2>Email Verification</h2>
        <p>Enter the 6-digit code sent to your email:</p>
        <Link href="/login">Send a code again</Link>

        <form style={{ width: '100%', display: 'flex', flexDirection: 'column', alignItems: 'center' }}>
          <div className="d-flex justify-content-center gap-2">
            {code.map((value, index) => (
              <input
                key={index}
                id={`code-${index}`}
                type="text"
                className="form-control"
                style={{ width: '40px', textAlign: 'center' }}
                maxLength={1}
                value={value}
                onChange={e => handleInput(index, e)}
              />
            ))}
          </div>

          <div className="mt-3">
            <button type="button" className="btn btn-primary" onClick={sendVerificationCode}>Verify</button>
          </div>
        </form>
      </div>
    </div>
  );}

export default VerifyEmail;

