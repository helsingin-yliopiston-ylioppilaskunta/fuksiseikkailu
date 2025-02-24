import "./Checkpoint.css";

const CheckpointListItem = () => {
    return (
        <li className="CheckpointListItem">
        </li>
    )
}

const CheckpointList = () => {
    return (
        <div className="CheckpointList">
            <h2>Checkpoints</h2>
            <ol>
                <CheckpointListItem />
            </ol>
        </div>
    )
}

export {
    CheckpointList,
    CheckpointListItem
}
