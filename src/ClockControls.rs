// export const ClockControls: FC<ControlsProps> = ({
//   setTimeInSeconds,
//   // timeInSeconds,
//   selectedProject,
// }) => {
//   const [intervalId, setIntervalId] = useState<number>(0);
//   const [isPlaying, setIsPlaying] = useState(false);
//   const { task, setTask } = useContext(TaskContext);
//   const createTaskMutation = useCreateTask();

//   const handlePlayButton = () => {
//     let interval: any = setInterval(() => {
//       setTimeInSeconds((previousState: number) => previousState + 1);
//     }, 1000);
//     setIntervalId(interval);
//     setIsPlaying(true);
//     setTask({ ...task, initial_time: new Date() });
//   };
//   const handleStopButton = () => {
//     clearInterval(intervalId);
//     setIsPlaying(false);
//     let endTime = new Date();

//     createTaskMutation.mutate({
//       ...task,
//       end_time: endTime,
//       project: selectedProject.id,
//     });
//     setTask({ name: '' });
//   };
//   const handleResetButton = () => {
//     clearInterval(intervalId);
//     setTimeInSeconds(0);
//     setIsPlaying(false);
//     setTask({ name: '' });
//   };

//   return (
//     <div className={styles.Controls}>
//       {isPlaying ? (
//         <button onClick={handleStopButton} data-testid='stop-button'>
//           <StopCircle className={styles['stop-btn']} size='32' />
//         </button>
//       ) : (
//         <button onClick={handlePlayButton} data-testid='play-button'>
//           <PlayCircle className={styles['play-btn']} size='32' />
//         </button>
//       )}
//       <button onClick={handleResetButton} data-testid='reset-button'>
//         <XCircle className={styles['reset-btn']} size='32' />
//       </button>
//     </div>
//   );
// };
