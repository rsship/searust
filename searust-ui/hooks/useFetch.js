import {useState, useCallback, useEffect} from "react";

export const useFetch = (url) => {
  const [data, setData] = useState(null);
  const [error, setError]= useState(null);
  const [loading, setLoading] = useState(false);

  const makeReq = async (val) => {
    try {
      setLoading(true);
      const resp = await fetch(url, {
        method: val == undefined ? "GET" : "POST", 
        body: val && val,
      }) 
      const rawData = await resp.json();
      setData(rawData);
    }catch(e) {
        setError(e);
    } finally {
        setLoading(false);
    }
  }
  

  return {
    data, error, loading, makeReq
  }

}

