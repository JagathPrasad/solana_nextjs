import { React } from 'react';
import styles from '../styles/signup.module.css';
import { useState } from 'react';

const Signup = ({ signup }) => {
    //console.log('props.signup', props.signup);
    return (
        <div className={styles.authContainer}>
            <h1 className={styles.title}>Sign up to use Tiktok</h1>
            <div className={styles.signupForm}>
                <div className={styles.inputFiled}>
                    <div className={styles.inputTitle}>
                        username
                    </div>
                    <div className={styles.inputContainer}>
                        <input className={styles.input} type="text" />
                    </div>
                </div>
                <div className={styles.inputFiled}>
                    <div className={styles.inputTitle}>
                        Profile Image
                    </div>
                    <div className={styles.inputContainer}>
                        <input className={styles.input} type="text" />
                    </div>
                </div>
            </div>
            <div className={styles.loginButton}>Signup</div>
        </div>
    );
}

export default Signup;