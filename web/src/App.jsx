import { useState } from 'react'
import './App.css'
import Bar from './Bar.jsx'

const fundingAmount = 100_000;
const mintAmount = 10_000;

const states = {
  Funding: 'funding',
  Submitted: 'submitted',
}

function App() {
  const [rawTx, setRawTx] = useState('');
  const [state, setState] = useState(states.Funding);
  const [note, setNote] = useState(undefined);

  const [ourAmount, setOurAmount] = useState(fundingAmount);

  return (
    <div style={{ 
      padding: '20px', 
      textAlign: 'center',
      display: 'flex',
      flexDirection: 'column',
    }}>
      {state === states.Funding && (
        <>
              <h2>Funding address</h2>
      <textarea 
        disabled={true}
        value={"bc1"}
        placeholder=""
        style={{
          width: '100%',
          maxWidth: '600px',
          padding: '15px',
          fontSize: '16px',
          border: '2px solid #ccc',
          borderRadius: '8px',
        }}
      />
              <div style={{ height: '50px' }}></div>
      <h2>Enter Transaction</h2>
      <textarea 
        value={rawTx}
        onChange={(e) => setRawTx(e.target.value)}
        placeholder="Enter raw transaction here..."
        style={{
          width: '500px',
          maxWidth: '600px',
          height: '200px',
          fontSize: '16px',
          padding: '15px',
          border: '2px solid #ccc',
          borderRadius: '8px',
          resize: 'vertical'
        }}
      />
      <div style={{ height: '30px' }}></div>
      <button 
        onClick={() => {
          setState(states.Submitted);
        }}
        style={{
          padding: '12px 24px',
          fontSize: '16px',
          backgroundColor: '#007bff',
          color: 'white',
          border: 'none',
          borderRadius: '8px',
          cursor: 'pointer',
          maxWidth: '200px',
          alignSelf: 'center'
        }}
        onMouseOver={(e) => e.target.style.backgroundColor = '#0056b3'}
        onMouseOut={(e) => e.target.style.backgroundColor = '#007bff'}
      >
        Submit Transaction
      </button>
        </>
      )}
      {state === states.Submitted && (
        <>
          <h2>Create ecash</h2>
      <Bar totalAmount={fundingAmount} ourAmount={ourAmount} />
                <button 
        onClick={() => {
          setOurAmount(ourAmount - mintAmount);
        }}
        style={{
          padding: '12px 24px',
          fontSize: '16px',
          backgroundColor: '#007bff',
          color: 'white',
          border: 'none',
          borderRadius: '8px',
          cursor: 'pointer',
          maxWidth: '200px',
          alignSelf: 'center'
        }}
        onMouseOver={(e) => e.target.style.backgroundColor = '#0056b3'}
        onMouseOut={(e) => e.target.style.backgroundColor = '#007bff'}
      >
        {mintAmount} sats
      </button>
      <div style={{ height: '30px' }}></div>

      {note !== undefined && (
        <>
          <h2>Note</h2>
          <textarea 
            disabled={true}
            value={note}
            style={{
              width: '100%',
              maxWidth: '600px',
              padding: '15px',
              fontSize: '16px',
              border: '2px solid #ccc',
              borderRadius: '8px',
            }}
          />
        </>
      )}
        </>
      )}
    </div>
  )
}

export default App
