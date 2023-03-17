import React from "react";
import { ChakraProvider } from '@chakra-ui/react'
import * as ReactDOM from 'react-dom/client'
import App from "./App";
// import "./style.css";

const rootElement = document.getElementById('root')
ReactDOM.createRoot(rootElement as HTMLElement).render(
  <React.StrictMode>
    <ChakraProvider>
      <App />
    </ChakraProvider>
  </React.StrictMode>,
)