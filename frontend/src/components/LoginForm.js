import React from "react";
import Container from "@mui/material/Container";
import Typography from "@mui/material/Typography";
import Box from '@mui/material/Box';
import TextField from '@mui/material/TextField';
import Button from '@mui/material/Button';
import login from "../scripts/api/login";

function LoginForm(){
    const [values, setValues] = React.useState({
        username: '',
        password: '',
    });

    const handleChange = (event) => {
        setValues({
            ...values,
            [event.target.name]: event.target.value,
        });
    };

    const submit = () => {
        login(values.username, values.password);
    }
    return(
        <Container component="main" maxWidth="xs">
            <Box
                component="form"
                sx={{
                    marginTop: 8,
                    display: 'flex',
                    flexDirection: 'column',
                    alignItems: 'center',
                }}
            >
                <Box>
                    <TextField
                        margin="normal"
                        required
                        fullWidth
                        id="username"
                        label="ユーザー名"
                        name="username"
                        autoComplete="username"
                        autoFocus
                        value={values.username}
                        onChange={handleChange}
                    />
                    <TextField
                        margin="normal"
                        required
                        fullWidth
                        name="password"
                        label="パスワード"
                        type="password"
                        id="password"
                        autoComplete="current-password"
                        value={values.password}
                        onChange={handleChange}
                    />
                    <Button
                        type="submit"
                        fullWidth
                        variant="contained"
                        color="primary"
                        onClick={submit}
                    >
                        ログイン
                    </Button>
                </Box>
            </Box>
        </Container>
    )
}

export default LoginForm;