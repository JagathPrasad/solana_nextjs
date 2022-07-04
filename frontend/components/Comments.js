import React, { useState, useEffect } from 'react';
import styles from '../styles/comment.module.css';
import { useWallet } from '@solana/wallet-adapter-react';
import {CommentCard} from './CommentCard';
const Comments = ({
    address,
    onHide,
    createComment,
    index,
    getComments,
    commentCount
}) => {
    const [comments, setComments] = useState([]);
    const [newComment, setNewComment] = useState('');

    useEffect(() => {
        getttingComments()
    }, [index])
    const getttingComments = async () => {
        let comments = await getComments(address, commentCount);
        comments.sort((a, b) => b.videoTime.toNumber() - a.videoTime.toNumber());
        setComments(comments);
    }
    const replyClicked = async () => {
        await createComment(address, commentCount, newComment);
        setNewComment('');
    }

    return (
        <div className={styles.wrapper}>
            <div className={styles.commentsHeader}>
                <p>{commentCount} comments count </p>
                <p className={styles.closeButton} onClick={onHide}>&times;</p>
            </div>
            {comments.map(comment => {
                return <CommentCard key={comment.index.toNumber()}
                    username={comment.commenterName}
                    comment={comment.text}
                    avatar={comment.commenterUrl}
                    timestamp={comment.videoTime} />
            })}
            <div className={styles.commentInputWrapper}>
                <input
                    type="text" onChange={e => setNewComment(e.target.value)}
                    value={newComment}
                    placeholder="leave a comment"
                />
                <button className={styles.button} onClick={replyClicked}>
                    reply
                </button>
            </div>
        </div>
    )
}
export default Comments;