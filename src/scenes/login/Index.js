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
import { useNavigate } from "react-router-dom";

function Login() {
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");
  const navigate = useNavigate();

  const handleSubmit = async (e) => {
    e.preventDefault();

    let user_id = null;
    let token = null;
    let role = null;

    try {
      const { data } = await axios.post("http://localhost:3001/user/login", {
        user_name: username,
        password: password,
      });

      console.log(JSON.stringify(data));

      user_id = data.user_id;
      token = data.token;
      role = data.role;
    } catch (error) {
      alert(error);
    }

    // persist all 3 and redirect to dashboard
    if (user_id == null || token == null || role == null) {
      // show error
      alert("login failed");
      return;
    }

    // set the local storage
    sessionStorage.setItem("user_id", user_id);
    sessionStorage.setItem("token", token);
    sessionStorage.setItem("role", role);

    let page_to_redirect =
      role === "reviewer" ? "/reviewer/home" : "/publisher/home";

    navigate(page_to_redirect);
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
                      type="password"
                      label="Password"
                      variant="standard"
                      value={password}
                      onChange={(e) => setPassword(e.target.value)}
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
