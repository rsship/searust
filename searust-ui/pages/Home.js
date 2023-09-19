import {useCallback, useState} from 'react';
import styles from '@/styles/Home.module.css';
import {useFetch} from '@/hooks/useFetch'


const Home = () => {
  const [inputValue, setInputValue] = useState(""); 

  const {data, error, loading, makeReq} = useFetch("http://127.0.0.1:6969/search")

  const handleSubmit = useCallback((e) => {
    e.preventDefault();
    makeReq(inputValue);
  }); 


  return (
    <>
      <div className={`${styles.title}`}>
        SeaRust!
      </div>
      <form onSubmit={handleSubmit}>
        <input 
           className={`${styles.input}`} 
           placeholder="Type for Search"
           onChange={(e) => {setInputValue(e.target.value)}}
           value={inputValue}
        >
        </input>
      </form>
      <div className={`${styles.container}`}>
        {data && data.map(([path], index) => {
            return (
              <div key={index}>{path}</div> 
            )
        })}
      </div>
    </>
  )

}


export default Home;
