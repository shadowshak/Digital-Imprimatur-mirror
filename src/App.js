import React from "react";
import { BrowserRouter as Router, Route, Routes } from "react-router-dom";
import CssBaseline from "@mui/material/CssBaseline";
import Document from "./scenes/document/Index";
import Home from "./scenes/home/Index";
import Login from "./scenes/login/Index";

function App() {
  return (
    <Router>
      <CssBaseline />
      <Routes>
        <Route path="/" element={<Home />} />
        <Route path="/login" element={<Login />} />
        <Route path="/document" element={<Document />} />
      </Routes>
    </Router>
  );
}

export default App;
