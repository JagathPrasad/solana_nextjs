import { React } from 'react';
import styles from '../styles/signup.module.css';
import { useState } from 'react';

const Signup = ({ signup }) => {
    //console.log('props.signup', props.signup);
    const [user_name, setUserName] = useState('');
    const [user_profile, setProfile] = useState('');
    const signupClicked = () => {
        console.log('signup clicked');
        console.log('signup user_name', user_name);
        console.log('signup user_profile', user_profile);
        signup(user_name, user_profile);
    }
    return (
        <div className={styles.authContainer}>
            <h1 className={styles.title}>Sign up to use Tiktok</h1>
            <div className={styles.signupForm}>
                <div className={styles.inputFiled}>
                    <div className={styles.inputTitle}>
                        username
                    </div>
                    <div className={styles.inputContainer}>
                        <input className={styles.input} type="text" onChange={e => setUserName(e.target.value)} />
                    </div>
                </div>
                <div className={styles.inputFiled}>
                    <div className={styles.inputTitle}>
                        Profile Image
                    </div>
                    <div className={styles.inputContainer}>
                        <input className={styles.input} type="text" onChange={e => setProfile(e.target.value)} />
                    </div>
                </div>
            </div>
            <div className={styles.loginButton} onClick={() => signupClicked()}>Signup</div>
        </div>
    );
}

export default Signup;