import React, { useState } from "react";
import {
  Box,
  Card,
  Grid,
  CardHeader,
  IconButton,
  CardContent,
  TextField,
  Button,
} from "@mui/material";
import ArrowBackIcon from "@mui/icons-material/ArrowBack";
import PlaceholderLogo from "../../assets/placeholderlogo.png";
import axios from "axios";

function Login() {
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");
  const [role, setRole] = useState("publisher");

  const handleSubmit = async (e) => {
    e.preventDefault();

    try {
      const response = await axios.post("http://localhost:3000/user/login", {
        user_name: username,
        password: password,
        role: role,
      });

      alert("Login successful!");

      console.log(response.data);
    } catch (error) {
      alert(error)
    }
  };

  return (
    <Box height="100vh" sx={{ backgroundColor: "#dee6ed" }}>
      <Grid
        container
        direction="column"
        alignItems="center"
        justifyContent="center"
        style={{ minHeight: "100vh" }}
      >
        <Grid item>
          <Card sx={{ p: "10px" }}>
            <CardHeader
              sx={{ mb: "-10px" }}
              avatar={
                <IconButton aria-label="back">
                  <ArrowBackIcon />
                </IconButton>
              }
            />
            <CardContent>
              <form onSubmit={handleSubmit}>
                <Grid
                  container
                  direction="column"
                  alignItems="center"
                  justifyContent="center"
                >
                  <Grid item sx={{ mb: "30px" }}>
                    <img src={PlaceholderLogo} alt="logo" />
                  </Grid>
                  <Grid item>
                    <TextField
                      sx={{ minWidth: "300px" }}
                      id="username"
                      label="Email, Phone, or Username"
                      variant="standard"
                      value={username}
                      onChange={(e) => setUsername(e.target.value)}
                    />
                  </Grid>
                  <Grid item sx={{ mt: "10px" }}>
                    <TextField
                      sx={{ minWidth: "300px" }}
                      id="password"
                      label="Password"
                      variant="standard"
                      value={password}
                      onChange={(e) => setPassword(e.target.value)}
                    />
                  </Grid>
                  <Grid item sx={{ mt: "10px" }}>
                    <TextField
                      sx={{ minWidth: "300px" }}
                      id="role"
                      label="Role"
                      variant="standard"
                      value={role}
                      onChange={(e) => setRole(e.target.value)}
                    />
                  </Grid>
                  <Grid
                    item
                    container
                    sx={{ width: "100%" }}
                    justifyContent="flex-end"
                  >
                    <Grid item>
                      <Button
                        sx={{
                          m: 0,
                          p: 0,
                          textTransform: "none",
                          fontSize: "0.75rem",
                          fontWeight: "bold",
                        }}
                      >
                        Forgot Username or Password?
                      </Button>
                    </Grid>
                  </Grid>
                  <Grid item sx={{ mt: "40px", width: "100%" }}>
                    <Button variant="contained" fullWidth type="submit">
                      Sign In
                    </Button>
                  </Grid>
                </Grid>
              </form>
            </CardContent>
          </Card>
        </Grid>
      </Grid>
    </Box>
  );
}

export default Login;
