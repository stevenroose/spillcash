function Bar({ totalAmount, ourAmount }) {
  const percentage = totalAmount > 0 ? (ourAmount / totalAmount) * 100 : 0;
  
  return (
    <div style={{ 
      width: '100%', 
      maxWidth: '600px', 
      margin: '20px auto',
      alignSelf: 'center'
    }}>
      <div style={{ 
        display: 'flex', 
        justifyContent: 'space-between', 
        marginBottom: '8px',
        fontSize: '14px',
        color: '#666'
      }}>
        <span>Our amount: {ourAmount.toLocaleString()} sats</span>
        <span>Total funding: {totalAmount.toLocaleString()} sats</span>
      </div>
      
      <div style={{
        width: '100%',
        height: '30px',
        backgroundColor: '#e0e0e0',
        borderRadius: '15px',
        overflow: 'hidden',
        position: 'relative',
        border: '2px solid #ccc'
      }}>
        <div style={{
          width: `${percentage}%`,
          height: '100%',
          backgroundColor: '#28a745',
          borderRadius: '13px',
          transition: 'width 0.3s ease',
          position: 'relative'
        }}>
          {percentage > 15 && (
            <span style={{
              position: 'absolute',
              right: '10px',
              top: '50%',
              transform: 'translateY(-50%)',
              color: 'white',
              fontSize: '12px',
              fontWeight: 'bold'
            }}>
              {percentage.toFixed(1)}%
            </span>
          )}
        </div>
        {percentage <= 15 && percentage > 0 && (
          <span style={{
            position: 'absolute',
            left: `${percentage}%`,
            marginLeft: '10px',
            top: '50%',
            transform: 'translateY(-50%)',
            color: '#666',
            fontSize: '12px',
            fontWeight: 'bold'
          }}>
            {percentage.toFixed(1)}%
          </span>
        )}
      </div>
    </div>
  );
}

export default Bar;