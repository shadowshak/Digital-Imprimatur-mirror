import React from "react";
import {
  Box,
  AppBar,
  Toolbar,
  IconButton,
  Typography,
  Tabs,
  Tab,
  List,
  ListItem,
  Card,
  CardContent,
  CardActions,
  CardHeader,
  Grid,
} from "@mui/material";
import MenuIcon from "@mui/icons-material/Menu";
import VisibilityIcon from "@mui/icons-material/Visibility";
import EditIcon from "@mui/icons-material/Edit";
import DeleteIcon from "@mui/icons-material/Delete";
import axios from "axios";

function Home({ role }) {
  const [submissions, setSubmissions] = React.useState([])

  const [value, setValue] = React.useState(1);

  const handleChange = (event, newValue) => {
    setValue(newValue);
  };

  const getSubmissions = async () => {
    const token = sessionStorage.getItem("token");

    const { data } = await axios.post("http://localhost:3000/user/submissions", {
      token
    });

    console.log(data);

    const responseSubmissions = data.submissions;
    const submissions = responseSubmissions.map(({
      id,
      meta: {
        name,
        author,
        description,
        creation,
        last_update,
        status,
      }
    }) => {
      return {
        id,
        name,
        author,
        description,
        creation,
        status
      }
    });

    setSubmissions(submissions)
  };

  React.useEffect(() => {
    try {
      getSubmissions();
    }
    catch(error) {
      console.log(JSON.stringify(error));
    }
  }, []);

  return (
    <>
      <AppBar>
        <Toolbar>
          <IconButton
            size="large"
            edge="start"
            color="inherit"
            aria-label="menu"
            sx={{ mr: 2 }}
          >
            <MenuIcon />
          </IconButton>
          <Typography variant="h6" component="div" sx={{ flexGrow: 1 }}>
            My Documents
          </Typography>
        </Toolbar>
      </AppBar>
      <Toolbar />
      <Grid
        container
        direction="column"
        alignItems="center"
        justifyContent="center"
        sx={{
          backgroundColor: "#dee6ed",
        }}
      >
        <Grid item>
          <Box>
            <Tabs
              value={value}
              onChange={handleChange}
              aria-label="Select Status Filter"
            >
              <Tab label="Completed" sx={{ width: "250px" }} />
              <Tab label="In Progress" sx={{ width: "250px" }} />
              <Tab
                label={role === "publisher" ? "Needs Attention" : "Todo"}
                sx={{ width: "250px" }}
              />
            </Tabs>
            <Box sx={{ overflow: "auto" }}>
              <List>
                {
                submissions.map((submission) => {
                  return (
                  <ListItem>
                    <SubmissionCard title={submission.name}
                                    author={submission.author}
                                    description={submission.description}
                                    publish_date={submission.creation}
                                    role={role}/>
                  </ListItem>
                  )
                })
                }
              </List>
            </Box>
          </Box>
        </Grid>
      </Grid>
    </>
  );
}

function SubmissionCard({
  title,
  author,
  description,
  publish_date,
  role,
}) {
  return (<Card sx={{ minWidth: "100vh" }}>
            <CardHeader
              action={
                <>
                  <IconButton aria-label="View">
                    <VisibilityIcon sx={{ color: "#1976d2" }} />
                  </IconButton>
                  <IconButton aria-label="Edit">
                    <EditIcon sx={{ color: "#1976d2" }} />
                  </IconButton>
                  {role === "publisher" && (
                    <IconButton aria-label="Delete">
                      <DeleteIcon sx={{ color: "#1976d2" }} />
                    </IconButton>
                  )}
                </>
              }
              title={title}
            />
            <CardContent>
              <Typography sx={{ mt: -4 }} color="text.secondary">
                {author}
              </Typography>
              <Typography sx={{ mb: 1.5 }} color="text.secondary">
                {publish_date}
              </Typography>
              <Typography
                variant="body2"
                color="text.secondary"
                sx={{ fontStyle: "italic" }}
              >
                {description}
              </Typography>
            </CardContent>
            <CardActions></CardActions>
          </Card>)
}

export default Home;
