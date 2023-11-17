"use client"

import React from 'react'
import Header from './Header'
import { ConfigProvider } from 'antd'
import axios from 'axios'
import { Toaster } from 'react-hot-toast';

function LayoutProvider({children}: {children: React.ReactNode}) {
  axios.defaults.validateStatus = function(status) {
    return status >= 200 && status < 500;
  };

  return (
    <html lang="en">
      <body>
        <Header/>
        <Toaster />
        <ConfigProvider
          theme={{
            token: {
              colorPrimary: "D3D3D3"
            }
          }}
        >
          {children}
        </ConfigProvider>
      </body>
    </html>
  )
}

export default LayoutProvider
